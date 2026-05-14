//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 919/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk919<F: Float>(t184: F, t563: F, t811: F, t3899: F, t571: F, t6194: F, t4738: F, t4946: F, t2565: F, t3783: F, t519: F, t2539: F, t3762: F, t10313: F, t2553: F, t518: F, t6610: F) -> (F, F, F, F, F, F, F) {
    let t18555 = t811 * t563 * t184;
    let t18575 = t571 * t3899 * t6194;
    let t18584 = t4738 * t4946;
    let t18593 = t519 * t3783 * t2565;
    let t18596 = t571 * t3762 * t2539;
    let t18599 = t519 * t10313 * t2553;
    let t18608 = t6610 * t518;
    (t18555, t18575, t18584, t18593, t18596, t18599, t18608)
}
