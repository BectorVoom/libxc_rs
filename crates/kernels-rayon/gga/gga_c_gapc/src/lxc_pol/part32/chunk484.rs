//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 484/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk484(t2655: f64, t2657: f64, t2661: f64, t2664: f64, t2671: f64, t2676: f64, t2679: f64, t2682: f64, t2685: f64, t2690: f64, t2695: f64, t2698: f64, t2703: f64, t2707: f64, t2713: f64, t2718: f64, t2722: f64, t2725: f64, t2728: f64, t2732: f64, t2737: f64, t321: f64, t326: f64, t886: f64, t890: f64, t893: f64, t904: f64, t917: f64, t925: f64, t934: f64, t940: f64, t946: f64) -> f64 {
    let t2740 = 0.28180301985989535023e-7_f64 * t2655 * t2657 - 0.50104576931089393271e-7_f64 * t2661 * t2657 + 0.88531029695126583729e-7_f64 * t2655 * t2664 - 0.15740817079793506587e-6_f64 * t2661 * t2664 + 0.27801896084645508334e-2_f64 * t321 * t2671 - 0.11594181388521408695e-4_f64 * t2676 * t2679 - 0.54106179813099907242e-4_f64 * t934 * t2682 - 0.10305939012019029951e-5_f64 * t940 * t2685 + 0.18323959563369835253e-5_f64 * t946 * t2685 + 0.7324140771837707598e-5_f64 * t2690 * t2695 + 0.60073333102343402209e-5_f64 * t2698 * t2703 - 0.27801896084645508334e-2_f64 * t2707 * t893 - 0.27801896084645508334e-2_f64 * t886 * t925 - 0.40544431790108032986e-3_f64 * t917 * t2713 - 0.13900948042322754167e-2_f64 * t321 * t2718 + 0.27801896084645508334e-2_f64 * t2722 * t2725 + 0.6487109086417285278e-2_f64 * t890 * t2728 - 0.37073828428874785365e-3_f64 * t904 * t2732 + 0.11594181388521408695e-4_f64 * t326 * t2737;
    t2740
}
