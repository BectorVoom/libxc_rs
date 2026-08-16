//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1268;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta384<F: Float>(t1043: F, t12050: F, t357: F, t19450: F, t6244: F, t999: F, t1082: F, t6234: F, t993: F, t225: F, t18902: F, t19025: F, t19027: F, t19029: F, t19031: F, t19048: F, t19051: F, t19053: F, t19055: F, t19058: F, t19060: F, t19062: F, t19079: F, t19081: F, t19084: F, t19130: F, t19132: F, t3011: F, t6205: F, t4733: F, t981: F, t15258: F, t4732: F, t4719: F, t4729: F, t19136: F, t19143: F, t19145: F, t19149: F, t19152: F, t19252: F, t19258: F, t19315: F, t19317: F, t19320: F, t19323: F, t19326: F, t19329: F, t19333: F, t19337: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19453, t19456, t19457, t19462, t19463, t19466) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1268::<F>(t1043, t12050, t357, t19450, t6244, t999, t1082, t6234, t993, t225, t18902, t19025, t19027, t19029, t19031, t19048, t19051, t19053, t19055, t19058, t19060, t19062, t19079, t19081, t19084, t19130, t19132);
        let (t19470, t19473, t19475, t19476) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1269::<F>(t3011, t6205, t4733, t981, t15258, t4732, t4719, t4729, t19136, t19143, t19145, t19149, t19152, t19252, t19258, t19315, t19317, t19320, t19323, t19326, t19329, t19333, t19337);
    (t19453, t19456, t19457, t19462, t19463, t19466, t19470, t19473, t19475, t19476)
}
