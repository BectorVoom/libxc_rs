//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 743/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk743<F: Float>(t4522: F, t6733: F, t1996: F, t4479: F, t2010: F, t4475: F, t1325: F, t3965: F, t3974: F, t4488: F, t4506: F, t4948: F, t6690: F, t6693: F, t6697: F, t6700: F, t6703: F, t6706: F, t6708: F, t6713: F, t6717: F, t6720: F, t6725: F, t6730: F, t6734: F) -> (F, F, F, F) {
    let t6737 = t4522 * t6733;
    let t6740 = t4479 * t1996;
    let t6743 = t4475 * t2010;
    let t6746 = -F::new(16.0) / F::new(45.0) * t6690 - F::new(8.0) / F::new(15.0) * t1325 * t6693 + F::new(8.0) / F::new(135.0) * t6697 + F::new(8.0) / F::new(81.0) * t6700 + F::new(8.0) / F::new(135.0) * t6703 + F::new(8.0) / F::new(81.0) * t6706 + F::new(16.0) / F::new(135.0) * t6708 - t4948 + F::new(16.0) / F::new(45.0) * t4488 * t6713 + F::new(16.0) / F::new(45.0) * t4488 * t6717 - F::new(8.0) / F::new(27.0) * t4488 * t6720 - F::new(16.0) / F::new(45.0) * t3974 * t6725 + F::new(16.0) / F::new(45.0) * t4506 * t6730 + F::new(16.0) / F::new(45.0) * t4506 * t6734 - F::new(8.0) / F::new(27.0) * t4506 * t6737 - F::new(16.0) / F::new(45.0) * t3965 * t6740 - F::new(16.0) / F::new(45.0) * t3974 * t6743;
    (t6737, t6740, t6743, t6746)
}
