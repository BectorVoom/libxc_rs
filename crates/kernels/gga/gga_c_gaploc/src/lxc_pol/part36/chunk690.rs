//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 690/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk690<F: Float>(t12830: F, t9074: F, t12428: F, t3152: F, t988: F, t2268: F, t3340: F, t894: F, t2765: F, t3137: F, t12821: F, t12823: F, t12824: F, t12825: F, t12828: F, t12829: F) -> (F, F, F, F) {
    let t12831 = t9074 * t12830;
    let t12832 = F::new(0.71137516589190373998e-2) * t12831;
    let t12833 = F::new(0.71137516589190373998e-2) * t12428;
    let t12834 = t3152 * t988;
    let t12836 = F::new(0.28455006635676149599e-1) * t2268 * t12834;
    let t12837 = t894 * t3340;
    let t12838 = t2268 * t12837;
    let t12840 = t2765 * t3137;
    let t12842 = F::new(0.85365019907028448797e-1) * t2268 * t12840;
    let t12843 = -F::new(0.23712505529730124666e-2) * t12821 - t12823 + t12824 + t12825 + t12828 + t12829 - t12832 - t12833 + t12836 + F::new(0.56910013271352299198e-1) * t12838 - t12842;
    (t12834, t12837, t12840, t12843)
}
