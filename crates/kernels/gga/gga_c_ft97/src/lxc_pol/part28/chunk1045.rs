//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1045/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1045<F: Float>(t136356: F, t136403: F, t136458: F, t136566: F, t136666: F, t136736: F, t136920: F, t136926: F, t136935: F, t145171: F, t22532: F, t22796: F, t25649: F, t25653: F, t25694: F, t25722: F, t25771: F, t25775: F, t25803: F, t3030: F, t32152: F, t34430: F, t36364: F, t36390: F, t37985: F, t6427: F, t7205: F, t92353: F) -> F {
    let t145297 = F::new(0.89080607335887169333e-3) * t136356 * t34430 - F::new(0.79202200203119310706e-5) * t136666 * t36364 * t25649 + F::new(0.79202200203119310706e-5) * t136926 * t36364 * t25653 - F::new(0.13784064983740990796e-3) * t136736 * t3030 - F::new(0.45497819271775541929e-4) * t136920 * t7205 * t145171 * t25694 - F::new(0.39601100101559655353e-5) * t22796 * t32152 * t25722 - F::new(0.17816121467177433867e-3) * t136566 * t25803 + F::new(0.21120586720831816188e-4) * t136935 * t25771 - F::new(0.59346127734643676855e-4) * t92353 * t36390 * t22532 * t25775 + F::new(0.28200083969358461042e-4) * t136458 - F::new(0.16779431174156321371e-9) * t37985 * t136403 * t6427;
    t145297
}
