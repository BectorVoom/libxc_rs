//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2148/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2148<F: Float>(t25207: F, t61203: F, t4433: F, t605: F, t892: F, t14749: F, t27159: F, t198: F, t7188: F, t11064: F, t7782: F, t1468: F, t1940: F, t2403: F, t25206: F, t25436: F, t25446: F, t25452: F, t27158: F, t27173: F, t27368: F, t27385: F, t51780: F, t7087: F, t7091: F, t7750: F, t98684: F, t98688: F, t98694: F, t98699: F, t98702: F, t98705: F) -> (F, F, F) {
    let t98709 = t25207 * t61203;
    let t98713 = t892 * t605 * t4433;
    let t98716 = t27159 * t14749;
    let t98719 = t198 * t7188;
    let t98722 = t7782 * t11064;
    let t98725 = -t1940 * t27368 * t25452 / F::new(2.0) + t98684 + t1940 * t25436 * t1468 / F::new(2.0) + F::new(3.0) * t25206 * t98688 + F::new(3.0) * t2403 * t7087 * t27173 - F::new(3.0) * t25206 * t98694 + F::new(3.0) * t51780 * t7750 + F::new(3.0) * t27158 * t98699 - t1940 * t7091 * t98702 - t1940 * t7091 * t98705 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t25206 * t98709 + F::new(6.0) * t27158 * t98713 + F::new(6.0) * t27158 * t98716 + F::new(2.0) * t98719 * t27385 + t1940 * t98722 * t25446;
    (t98719, t98722, t98725)
}
