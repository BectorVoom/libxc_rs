//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 975/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk975<F: Float>(t1559: F, t18627: F, t2747: F, t18444: F, t6035: F, t10770: F, t18469: F, t1544: F, t2723: F, t18426: F, t14846: F, t14850: F, t14866: F, t18403: F, t18411: F, t18416: F, t18420: F, t18424: F, t18433: F, t18442: F, t2745: F, t4362: F) -> (F, F, F, F, F) {
    let t23323 = t2747 * t18627 * t1559;
    let t23327 = t2747 * t18444 * t6035;
    let t23331 = t10770 * t18469 * t1559;
    let t23334 = t2723 * t1544;
    let t23336 = t2747 * t18426 * t23334;
    let t23339 = -F::cast_from(0.91464571985215438873e-3_f64) * t14846 - F::cast_from(0.22866142996303859718e-3_f64) * t14850 - F::cast_from(0.15246000842785598468e-3_f64) * t18403 + F::cast_from(0.21437009059034868486e-4_f64) * t18411 - F::cast_from(0.42874018118069736972e-4_f64) * t18416 + F::cast_from(0.21437009059034868486e-4_f64) * t18420 + F::cast_from(0.76230004213927992338e-3_f64) * t18424 + F::cast_from(0.76230004213927992337e-4_f64) * t18433 - F::cast_from(0.17149607247227894789e-3_f64) * t18442 - F::cast_from(0.68026775414003982663e-1_f64) * t14866 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t23323 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t23327 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t23331 - F::cast_from(0.51448821741683684367e-2_f64) * t4362 * t23336;
    (t23323, t23327, t23331, t23336, t23339)
}
