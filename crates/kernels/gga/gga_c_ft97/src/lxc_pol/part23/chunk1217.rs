//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1217/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1217<F: Float>(t1096: F, t27547: F, t24330: F, t30717: F, t6808: F, t65782: F, t30853: F, t96539: F, t108518: F, t108688: F, t108697: F, t108733: F, t108797: F, t108817: F, t108826: F, t109008: F, t109064: F, t109231: F, t109266: F, t109272: F, t109273: F, t122778: F, t122836: F, t122840: F, t13520: F, t17807: F, t17818: F, t17859: F, t17864: F, t17865: F, t18003: F, t231: F, t24311: F, t24324: F, t27506: F, t27515: F, t27534: F, t27561: F, t27651: F, t27652: F, t33372: F, t35409: F, t3746: F, t3766: F, t3773: F, t4939: F, t52358: F, t6027: F, t6035: F, t6045: F, t6785: F, t688: F, t79605: F, t96450: F, t96716: F) -> (F, F, F) {
    let t122932 = t1096 * t27547;
    let t122957 = t6808 * t24330 * t30717;
    let t122959 = t1096 * t65782;
    let t122972 = t30853 * t96539;
    let t122983 = 0.36061544906567819424e-6 * t52358 * t4939 * t17818 * t3773 * t6027 * t688 - t108688 - 0.17816121467177433866e-2 * t109231 * t122840 * t122932 - 0.17816121467177433866e-3 * t108518 * t109266 * t122836 + 0.51074886703703703704e-1 * t27651 * t108817 * t27652 * t3746 - 0.10338048737805743097e-3 * t109064 * t6785 * t17864 + t108697 - 0.34526011664076264185e-5 * t108733 - 0.3959138103817207526e-3 * t108797 * t27534 - 0.45967398033333333333e0 * t3766 * t96450 * t6035 * t35409 * t17859 + 0.474190451827401039e-1 * t33372 * t18003 - 0.12768721675925925926e-1 * t122957 + 0.17816121467177433866e-3 * t96716 * t109266 * t122959 + 0.59346127734643676855e-4 * t109272 * t109273 * t122959 - 0.20411767610277765552e-7 * t17807 * t109008 * t27561 + 0.1721820212247325051e-5 * t13520 * t24311 * t122778 - 0.17263005832038132092e-5 * t122972 - 0.22983699016666666666e0 * t24324 * t6045 * t231 * t79605 + 0.61289864044444444446e0 * t24324 * t27506 * t27515 + 0.27568129967481981592e-3 * t108826 * t17865;
    (t122932, t122959, t122983)
}
