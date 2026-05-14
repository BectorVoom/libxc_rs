//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1199/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1199<F: Float>(t34989: F, t34992: F, t34995: F, t35001: F, t35003: F, t35005: F, t35007: F, t35010: F, t35013: F, t35016: F, t35019: F, t35024: F, t35027: F, t35031: F, t35036: F, t35039: F) -> (F, F, F, F, F, F) {
    let t37156 = 0.12141398358188788626e-5 * t34989;
    let t37157 = 0.10793703140429833089e-5 * t34992;
    let t37170 = 0.21587406280859666178e-5 * t34995 - 0.24375961217880947793e-4 * t35001 + 0.9275345110817126956e-4 * t35003 - 0.9275345110817126956e-4 * t35005 + 0.99044544404633838508e-5 * t35007 - 0.45020247456651744776e-6 * t35010 - 0.77294542590142724634e-6 * t35013 - 0.13259557375557346398e-6 * t35016 + 0.80045999977926802213e-8 * t35019 - 0.49755503537412447748e-6 * t35024 + 0.61551119569641057312e-8 * t35027;
    let t37172 = 0.40483072916666666669e-4 * t35031;
    let t37174 = 0.50680539737635041234e-3 * t35036;
    let t37175 = 0.25301920572916666668e-5 * t35039;
    (t37156, t37157, t37170, t37172, t37174, t37175)
}
