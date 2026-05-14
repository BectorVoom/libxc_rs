//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 756/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk756<F: Float>(t3743: F, t4673: F, t2192: F, t62: F, t694: F, t891: F, t133: F, t7766: F, t742: F, t7704: F, t947: F, t131: F, t155: F, t205: F, t2341: F, t3758: F, t4633: F, t4643: F, t4649: F, t4652: F, t7768: F, t7776: F, t8096: F, t8101: F, t8524: F, t976: F) -> (F, F) {
    let t8731 = t4673 * t3743;
    let t8732 = t62 * t2192;
    let t8734 = t891 * t8732 * t694;
    let t8743 = t133 * t7766;
    let t8744 = t742 * t8743;
    let t8747 = t947 * t7704;
    let t8748 = t131 * t8747;
    let t8757 = 0.29617398950766044 * t8731 * t8734 - 7.108175748183851 * t3758 * t8524 - 19.489173774580152 * t155 * t7768 - 19.489173774580152 * t155 * t7776 - t4633 + 2.3693919160612835 * t205 * t8744 + 2.3693919160612835 * t205 * t8748 - t4643 - t4649 - t4652 - 19.489173774580152 * t155 * t8096 - 19.489173774580152 * t155 * t8101 + 19.489173774580152 * t976 * t2341;
    (t8732, t8757)
}
