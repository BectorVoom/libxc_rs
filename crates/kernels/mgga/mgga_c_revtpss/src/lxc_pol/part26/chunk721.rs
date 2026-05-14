//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 721/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk721<F: Float>(t10001: F, t10003: F, t1410: F, t3934: F, t3944: F, t9932: F, t9937: F, t9944: F, t9953: F, t9958: F, t9963: F, t9966: F, t9971: F, t9973: F, t9977: F, t9982: F, t9986: F, t9993: F, t9997: F) -> (F,) {
    let t10004 = t10001 * t10003;
    let t10006 = 0.21437009059034868486e-4 * t9932 - 0.42874018118069736972e-4 * t9937 - 0.25724410870841842183e-1 * t1410 * t9944 - t9953 - 0.12862205435420921092e-1 * t3934 * t9958 - 0.24009450146119052704e-1 * t9963 + 3.0 / 16.0 * t3944 * t9966 - 0.38115002106963996168e-4 * t9971 + 0.30011812682648815881e-2 * t9973 + 0.40656002247428262579e-3 * t9977 - 0.17149607247227894789e-3 * t9982 + 0.12862205435420921092e-1 * t1410 * t9986 - 0.12862205435420921092e-2 * t9993 * t9997 + 0.76230004213927992337e-4 * t10004;
    (t10006,)
}
