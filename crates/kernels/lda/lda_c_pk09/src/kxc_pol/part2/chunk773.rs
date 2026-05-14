//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 773/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk773<F: Float>(t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t4313: F, t4320: F, t4322: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F, t7896: F, t7900: F, t7904: F, t7908: F, t7913: F, t7917: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> (F, F) {
    let t9109 = 0.10188339589005964 * t3323 + 0.10188339589005964 * t3326 + t4313 + 4.596908415362055 * t7870 - 4.596908415362055 * t7875 + 4.596908415362055 * t7879 - 4.596908415362055 * t7884 + 4.596908415362055 * t7888 + 3.06460561024137 * t3424 + 3.06460561024137 * t3426 - 3.06460561024137 * t3428 + t4320 + t4322;
    let t9124 = 0.30565018767017893 * t7896 - 4.596908415362055 * t7900 - 4.596908415362055 * t7904 - 4.596908415362055 * t7908 + 6.8953626230430825 * t7913 + 4.596908415362055 * t7917 + 0.15282509383508946 * t7919 + 0.15282509383508946 * t7923 + 0.15282509383508946 * t7926 + 0.15282509383508946 * t7928 + 0.15282509383508946 * t7931 + 0.15282509383508946 * t7935 + 0.10188339589005964 * t7939 + 0.10188339589005964 * t7942;
    (t9109, t9124)
}
