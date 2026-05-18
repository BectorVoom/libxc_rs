//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1221/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1221<F: Float>(t26359: F, t9303: F, t10146: F, t2097: F, t25921: F, t25930: F, t26304: F, t26371: F, t7295: F, t7296: F, t94721: F, t94868: F, t96556: F, t96559: F, t96561: F, t96564: F, t96565: F, t96567: F, t96570: F, t96577: F, t96584: F, t96588: F) -> F {
    let t96591 = F::new(0.26019841438354088051e-2) * t9303 * t26359;
    let t96594 = F::new(0.16463622957338778996e-1) * t96556 + F::new(0.19514881078765566037e-2) * t96559 - F::new(0.39029762157531132076e-1) * t96561 - t96564 + F::new(0.57824187921367996415e-1) * t96565 + F::new(0.38554277296572111609e-1) * t96567 + F::new(0.32927245914677557992e-1) * t96570 - F::new(0.26020884564615598386e1) * t25930 * t26304 * t94721 - F::new(0.58544643236296698113e-1) * t96577 + F::new(0.8673628188205199462e0) * t7295 * t7296 * t2097 * t10146 - t96584 - F::new(0.26020884564615598386e1) * t25930 * t26304 * t94868 + F::new(0.77108554593144223218e-1) * t96588 + t96591 - F::new(0.78062653693846795158e1) * t25921 * t26371;
    t96594
}
