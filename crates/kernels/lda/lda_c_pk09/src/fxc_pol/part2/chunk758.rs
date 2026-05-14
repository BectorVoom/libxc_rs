//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 758/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk758<F: Float>(t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t3643: F, t3650: F, t3652: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F, t7896: F, t7900: F, t7904: F, t7908: F, t7913: F, t7917: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> (F, F) {
    let t8797 = 0.4266666666666667 * t3323 + 0.4266666666666667 * t3326 + t3643 + 19.250905149166083 * t7870 - 19.250905149166083 * t7875 + 19.250905149166083 * t7879 - 19.250905149166083 * t7884 + 19.250905149166083 * t7888 + 12.833936766110723 * t3424 + 12.833936766110723 * t3426 - 12.833936766110723 * t3428 + t3650 + t3652;
    let t8812 = 1.28 * t7896 - 19.250905149166083 * t7900 - 19.250905149166083 * t7904 - 19.250905149166083 * t7908 + 28.876357723749127 * t7913 + 19.250905149166083 * t7917 + 0.64 * t7919 + 0.64 * t7923 + 0.64 * t7926 + 0.64 * t7928 + 0.64 * t7931 + 0.64 * t7935 + 0.4266666666666667 * t7939 + 0.4266666666666667 * t7942;
    (t8797, t8812)
}
