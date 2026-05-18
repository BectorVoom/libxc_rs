//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 922/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk922<F: Float>(t5043: F, t5047: F, t5056: F, t5071: F, t5731: F, t5733: F, t5739: F, t9623: F, t9628: F, t9631: F, t9635: F, t9742: F, t9746: F, t9750: F, t9753: F, t9756: F) -> F {
    let t9758 = t5731 - F::new(11.879313099038017) * t5043 + t5733 + F::new(11.879313099038017) * t5047 - F::new(11.879313099038017) * t9623 + F::new(23.758626198076033) * t9628 - F::new(3.959771033012672) * t9631 - F::new(11.879313099038017) * t9635 - F::new(11.879313099038017) * t9742 - F::new(3.959771033012672) * t5056 - t5739 + F::new(3.959771033012672) * t5071 + F::new(11.879313099038017) * t9746 - F::new(11.879313099038017) * t9750 + F::new(3.959771033012672) * t9753 + F::new(11.879313099038017) * t9756;
    t9758
}
