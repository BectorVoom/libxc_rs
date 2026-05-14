//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 917/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk917<F: Float>(t1559: F, t18627: F, t2747: F, t18444: F, t6035: F, t10770: F, t18469: F, t1544: F, t2723: F, t18426: F, t14846: F, t14850: F, t14866: F, t18403: F, t18411: F, t18416: F, t18420: F, t18424: F, t18433: F, t18442: F, t2745: F, t4362: F) -> (F, F, F, F, F) {
    let t23323 = t2747 * t18627 * t1559;
    let t23327 = t2747 * t18444 * t6035;
    let t23331 = t10770 * t18469 * t1559;
    let t23334 = t2723 * t1544;
    let t23336 = t2747 * t18426 * t23334;
    let t23339 = -0.91464571985215438873e-3 * t14846 - 0.22866142996303859718e-3 * t14850 - 0.15246000842785598468e-3 * t18403 + 0.21437009059034868486e-4 * t18411 - 0.42874018118069736972e-4 * t18416 + 0.21437009059034868486e-4 * t18420 + 0.76230004213927992338e-3 * t18424 + 0.76230004213927992337e-4 * t18433 - 0.17149607247227894789e-3 * t18442 - 0.68026775414003982663e-1 * t14866 + 0.25724410870841842183e-2 * t2745 * t23323 + 0.25724410870841842183e-2 * t2745 * t23327 - 0.12862205435420921092e-1 * t2745 * t23331 - 0.51448821741683684367e-2 * t4362 * t23336;
    (t23323, t23327, t23331, t23336, t23339)
}
