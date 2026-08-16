//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 975/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk975(t1559: f64, t18627: f64, t2747: f64, t18444: f64, t6035: f64, t10770: f64, t18469: f64, t1544: f64, t2723: f64, t18426: f64, t14846: f64, t14850: f64, t14866: f64, t18403: f64, t18411: f64, t18416: f64, t18420: f64, t18424: f64, t18433: f64, t18442: f64, t2745: f64, t4362: f64) -> (f64, f64, f64, f64, f64) {
    let t23323 = t2747 * t18627 * t1559;
    let t23327 = t2747 * t18444 * t6035;
    let t23331 = t10770 * t18469 * t1559;
    let t23334 = t2723 * t1544;
    let t23336 = t2747 * t18426 * t23334;
    let t23339 = -0.91464571985215438873e-3_f64 * t14846 - 0.22866142996303859718e-3_f64 * t14850 - 0.15246000842785598468e-3_f64 * t18403 + 0.21437009059034868486e-4_f64 * t18411 - 0.42874018118069736972e-4_f64 * t18416 + 0.21437009059034868486e-4_f64 * t18420 + 0.76230004213927992338e-3_f64 * t18424 + 0.76230004213927992337e-4_f64 * t18433 - 0.17149607247227894789e-3_f64 * t18442 - 0.68026775414003982663e-1_f64 * t14866 + 0.25724410870841842183e-2_f64 * t2745 * t23323 + 0.25724410870841842183e-2_f64 * t2745 * t23327 - 0.12862205435420921092e-1_f64 * t2745 * t23331 - 0.51448821741683684367e-2_f64 * t4362 * t23336;
    (t23323, t23327, t23331, t23336, t23339)
}
