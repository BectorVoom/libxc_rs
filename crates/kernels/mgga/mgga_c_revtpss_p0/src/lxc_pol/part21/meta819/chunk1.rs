//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3022/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3022<F: Float>(t53310: F, t53351: F, t53377: F, t53395: F, t53425: F, t53455: F, t53490: F, t53528: F, t53549: F, t53581: F, t53617: F, t53645: F, t53682: F, t53716: F, t53759: F, t53785: F, t53816: F, t53844: F, t53883: F, t53920: F, t53954: F, t53987: F, t54013: F, t54049: F, t54083: F, t54110: F, t54149: F, t54176: F, t54195: F, t54224: F, t54275: F, t54308: F, t54346: F, t54389: F, t54418: F, t54455: F, t54495: F, t54526: F, t54559: F, t54589: F, t54622: F, t54653: F, t54684: F, t54712: F, t54735: F, t54770: F, t54806: F, t54843: F, t54880: F, t54904: F, t54945: F, t54977: F, t55016: F, t55039: F, t55069: F, t55096: F, t55140: F, t55163: F, t55198: F, t55237: F, t55271: F, t55303: F, t55338: F, t55371: F) -> F {
    let t55377 = t54945 + t54013 + t55237 + t54195 + t54418 + t54149 + t54735 + t54176 + t54880 + t55096 + t54770 + t54275 + t55039 + t53987 + t53844 + t55163 + t54559 + t55140 + t53645 + t54622 + t54110 + t54806 + t53883 + t54495 + t54346 + t53490 + t53528 + t54684 + t54224 + t53816 + t54526 + t54455 + t53310 + t54977 + t53549 + t53395 + t53617 + t54712 + t53455 + t53682 + t55198 + t53425 + t54843 + t54308 + t54083 + t54389 + t53785 + t55271 + t54589 + t55069 + t53954 + t54653 + t53377 + t53716 + t53920 + t55338 + t53351 + t53759 + t54049 + t55016 + t55303 + t54904 + t55371 + t53581;
    t55377
}
