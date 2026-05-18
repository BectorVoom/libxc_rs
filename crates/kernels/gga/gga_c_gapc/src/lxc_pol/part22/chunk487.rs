//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 487/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk487<F: Float>(t2655: F, t2657: F, t2661: F, t2664: F, t2671: F, t2676: F, t2679: F, t2682: F, t2685: F, t2690: F, t2695: F, t2698: F, t2703: F, t2707: F, t2713: F, t2718: F, t2722: F, t2725: F, t2728: F, t2732: F, t2737: F, t321: F, t326: F, t886: F, t890: F, t893: F, t904: F, t917: F, t925: F, t934: F, t940: F, t946: F) -> F {
    let t2740 = F::new(0.28180301985989535023e-7) * t2655 * t2657 - F::new(0.50104576931089393271e-7) * t2661 * t2657 + F::new(0.88531029695126583729e-7) * t2655 * t2664 - F::new(0.15740817079793506587e-6) * t2661 * t2664 + F::new(0.27801896084645508334e-2) * t321 * t2671 - F::new(0.11594181388521408695e-4) * t2676 * t2679 - F::new(0.54106179813099907242e-4) * t934 * t2682 - F::new(0.10305939012019029951e-5) * t940 * t2685 + F::new(0.18323959563369835253e-5) * t946 * t2685 + F::new(0.7324140771837707598e-5) * t2690 * t2695 + F::new(0.60073333102343402209e-5) * t2698 * t2703 - F::new(0.27801896084645508334e-2) * t2707 * t893 - F::new(0.27801896084645508334e-2) * t886 * t925 - F::new(0.40544431790108032986e-3) * t917 * t2713 - F::new(0.13900948042322754167e-2) * t321 * t2718 + F::new(0.27801896084645508334e-2) * t2722 * t2725 + F::new(0.6487109086417285278e-2) * t890 * t2728 - F::new(0.37073828428874785365e-3) * t904 * t2732 + F::new(0.11594181388521408695e-4) * t326 * t2737;
    t2740
}
