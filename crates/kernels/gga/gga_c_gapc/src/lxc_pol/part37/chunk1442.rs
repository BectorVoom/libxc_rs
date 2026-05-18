//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1442/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1442<F: Float>(t224: F, t38816: F, t38825: F, t38831: F, t38835: F, t12589: F, t12623: F, t12588: F, t12654: F, t987: F, t36095: F, t36100: F, t36103: F, t36105: F, t36109: F, t36111: F, t36113: F, t36116: F, t36119: F, t36270: F, t36271: F, t36275: F, t36283: F, t36285: F, t38537: F, t38556: F, t38689: F, t38692: F) -> (F, F, F, F, F) {
    let t38838 = t224 * (t38816 + t38825 + t38831 + t38835);
    let t38842 = F::new(2.0) * t12589;
    let t38843 = F::new(2.0) * t12623;
    let t38844 = F::new(2.0) * t12588;
    let t38863 = t987 * t12654;
    let t38891 = -t36095 + t38537 - t36100 - t36103 + t36105 + t38556 - t38689 - t36109 + F::new(2.0) * t38863 + t36111 - t36113 - t36116 + t36119 + t38692 - t36270 - t36271 - t36275 + t36283 - t36285;
    (t38838, t38842, t38843, t38844, t38891)
}
