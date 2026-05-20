//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1496/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1496<F: Float>(t1390: F, t828: F, t9995: F, t2482: F, t27: F, t4000: F, t221: F, t4004: F, t4019: F, t1410: F, t3934: F, t3944: F, t9932: F, t9937: F, t9944: F, t9953: F, t9958: F, t9963: F, t9966: F, t9971: F, t9973: F, t9977: F, t9982: F, t9986: F, t9993: F) -> (F, F, F, F, F) {
    let t9997 = t1390 * t828 * t9995;
    let t10001 = t2482 * t4000 * t27;
    let t10003 = t4019 * t221 * t4004;
    let t10004 = t10001 * t10003;
    let t10006 = F::cast_from(0.21437009059034868486e-4_f64) * t9932 - F::cast_from(0.42874018118069736972e-4_f64) * t9937 - F::cast_from(0.25724410870841842183e-1_f64) * t1410 * t9944 - t9953 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t9958 - F::cast_from(0.24009450146119052704e-1_f64) * t9963 + F::new(3.0) / F::new(16.0) * t3944 * t9966 - F::cast_from(0.38115002106963996168e-4_f64) * t9971 + F::cast_from(0.30011812682648815881e-2_f64) * t9973 + F::cast_from(0.40656002247428262579e-3_f64) * t9977 - F::cast_from(0.17149607247227894789e-3_f64) * t9982 + F::cast_from(0.12862205435420921092e-1_f64) * t1410 * t9986 - F::cast_from(0.12862205435420921092e-2_f64) * t9993 * t9997 + F::cast_from(0.76230004213927992337e-4_f64) * t10004;
    (t9997, t10001, t10003, t10004, t10006)
}
