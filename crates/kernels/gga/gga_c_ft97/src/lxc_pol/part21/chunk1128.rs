//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1128/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1128<F: Float>(t4491: F, t58: F, t16011: F, t1742: F, t5570: F, t100734: F, t100753: F, t101200: F, t101209: F, t101228: F, t115617: F, t15649: F, t15665: F, t15669: F, t15674: F, t15720: F, t15793: F, t2035: F, t22522: F, t22583: F, t22590: F, t22591: F, t22736: F, t22738: F, t22826: F, t25692: F, t25708: F, t25734: F, t29515: F, t373: F, t384: F, t401: F, t423: F, t428: F, t4431: F, t4474: F, t5790: F, t7202: F, t7889: F) -> (F, F) {
    let t115687 = t58 * t4491;
    let t115702 = t5570 * t1742 * t16011;
    let t115729 = 0.61277550024922479209e-6 * t22736 * t22738 * t29515 + 0.44455354858818847408e-2 * t22590 * t22591 * t115687 * t401 - 0.44455354858818847408e-2 * t7889 * t22591 * t115687 * t428 + 0.12768721675925925926e-1 * t22522 * t5570 * t423 * t4431 * t428 + 0.12768721675925925926e-1 * t25708 * t115702 + 0.27039520901431665706e-3 * t15793 * t101228 + t100734 + 0.59346127734643676855e-4 * t101209 * t101200 * t115617 + 0.74233839446572641111e-4 * t22583 * t25692 * t4431 * t373 * t384 - t100753 - 0.1054015240332537869e-3 * t7202 * t2035 * t5790 * t4474 - 0.38731446812548799881e-3 * t22826 * t15665 - 0.23238868087529279928e-3 * t25734 * t15649 + 0.77462893625097599762e-3 * t25734 * t15669 - 0.64507906339763927061e-5 * t25734 * t15720 + 0.23254900946437792e-1 * t22826 * t15674;
    (t115702, t115729)
}
