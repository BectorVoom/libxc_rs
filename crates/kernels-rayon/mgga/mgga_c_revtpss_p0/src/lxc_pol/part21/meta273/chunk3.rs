//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1496/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1496(t1390: f64, t828: f64, t9995: f64, t2482: f64, t27: f64, t4000: f64, t221: f64, t4004: f64, t4019: f64, t1410: f64, t3934: f64, t3944: f64, t9932: f64, t9937: f64, t9944: f64, t9953: f64, t9958: f64, t9963: f64, t9966: f64, t9971: f64, t9973: f64, t9977: f64, t9982: f64, t9986: f64, t9993: f64) -> (f64, f64, f64, f64, f64) {
    let t9997 = t1390 * t828 * t9995;
    let t10001 = t2482 * t4000 * t27;
    let t10003 = t4019 * t221 * t4004;
    let t10004 = t10001 * t10003;
    let t10006 = 0.21437009059034868486e-4_f64 * t9932 - 0.42874018118069736972e-4_f64 * t9937 - 0.25724410870841842183e-1_f64 * t1410 * t9944 - t9953 - 0.12862205435420921092e-1_f64 * t3934 * t9958 - 0.24009450146119052704e-1_f64 * t9963 + 3.0_f64 / 16.0_f64 * t3944 * t9966 - 0.38115002106963996168e-4_f64 * t9971 + 0.30011812682648815881e-2_f64 * t9973 + 0.40656002247428262579e-3_f64 * t9977 - 0.17149607247227894789e-3_f64 * t9982 + 0.12862205435420921092e-1_f64 * t1410 * t9986 - 0.12862205435420921092e-2_f64 * t9993 * t9997 + 0.76230004213927992337e-4_f64 * t10004;
    (t9997, t10001, t10003, t10004, t10006)
}
