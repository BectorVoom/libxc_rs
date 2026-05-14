//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1130/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1130<F: Float>(t10918: F, t11368: F, t11369: F, t11451: F, t11455: F, t23825: F, t25419: F, t25540: F, t25633: F, t25657: F, t25660: F, t25664: F, t25667: F, t25670: F, t25682: F, t25684: F, t2668: F, t2721: F, t2797: F, t3608: F, t7397: F, t7988: F, t7992: F, t8037: F, t8149: F, t8171: F, t914: F, t930: F) -> (F,) {
    let t25689 = 0.6058720680803250206e1 * t11368 * t11369 * t25419 - 0.93770531639908660928e4 * t11451 * t7988 - 0.16156588482142000549e2 * t8149 * t8171 + 0.46885265819954330464e4 * t11455 * t7992 + 0.15146801702008125515e1 * t25657 + 0.15146801702008125515e1 * t25660 + 0.11721316454988582616e4 * t25664 + 0.20195735602677500687e1 * t25667 - 0.58606582274942913081e3 * t25670 + 0.10431793787746509425e1 * t930 * t914 * t7397 * t23825 + 0.18545411178216016757e1 * t2797 * t8037 - 0.12117441361606500412e2 * t2721 * t3608 * t25633 + 0.6717427261115226305e-1 * t25682 - 0.33268896651293990656e3 * t25684 - 0.51620760404990155789e2 * t2668 * t25540 * t10918;
    (t25689,)
}
