//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 860/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk860<F: Float>(t18313: F, t18372: F, t41596: F, t590: F, t20535: F, t34688: F, t9537: F, t26796: F, t9282: F, t20671: F, t31037: F, t35101: F) -> (F, F, F, F) {
    let t42064 = F::new(0.61348681526273199482e1) * t18372 * t18313 * t41596 * t590;
    let t42066 = t20535 * t34688 * t9537;
    let t42067 = F::new(0.11502877786176224903e1) * t42066;
    let t42069 = F::new(0.47667319935800568892e0) * t26796 * t9282;
    let t42071 = t31037 * t20671 * t35101;
    (t42064, t42067, t42069, t42071)
}
