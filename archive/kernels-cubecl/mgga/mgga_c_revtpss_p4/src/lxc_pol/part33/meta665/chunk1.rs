//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2175/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2175<F: Float>(t1444: F, t6874: F, t22453: F, t94901: F, t108368: F, t25895: F, t108225: F, t14230: F, t25930: F, t25931: F, t27868: F, t27973: F, t27981: F, t3999: F, t6918: F, t7274: F, t7295: F, t7296: F, t75012: F, t7910: F, t94865: F, t94867: F, t97933: F, t98084: F, t98089: F, t98091: F, t98099: F) -> F {
    let t108448 = t6874 * t1444;
    let t108455 = t94901 * t22453;
    let t108464 = t25895 * t108368;
    let t108471 = -t94865 - F::cast_from(0.17347256376410398924e1_f64) * t108225 * t27981 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t25931 * t108448 + F::cast_from(0.8673628188205199462e0_f64) * t27868 * t25931 * t75012 + F::cast_from(0.19514881078765566037e-1_f64) * t108455 - t94867 - F::cast_from(0.45699670022203476294e-2_f64) * t98084 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t7274 * t6918 - F::cast_from(0.17347256376410398924e1_f64) * t97933 * t27973 + t98089 - t98091 - F::cast_from(0.14456046980341999104e-1_f64) * t108464 - F::cast_from(0.17347256376410398924e1_f64) * t27868 * t3999 * t7910 * t14230 - F::cast_from(0.4818682326780666368e-3_f64) * t98099;
    t108471
}
