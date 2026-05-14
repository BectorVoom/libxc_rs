//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1187/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1187<F: Float>(t1736: F, t373: F, t420: F, t22632: F, t25670: F, t5598: F, t100542: F, t100556: F, t100834: F, t101201: F, t11109: F, t11121: F, t1593: F, t1632: F, t1643: F, t1651: F, t1656: F, t18: F, t2248: F, t22558: F, t22583: F, t22585: F, t22598: F, t22652: F, t22767: F, t22799: F, t22820: F, t25626: F, t25692: F, t25693: F, t34434: F, t384: F, t423: F, t5537: F, t920: F, t92357: F, t92358: F, t92456: F, t92471: F, t92897: F, t929: F, t930: F, t93122: F, t93129: F, t93176: F, t93189: F, t93192: F, t93195: F) -> (F,) {
    let t101466 = t420 * t1736 * t373;
    let t101498 = 0.25537443351851851852e-1 * t5598 * t22632 * t25670;
    let t101503 = -0.81118562704294997117e-4 * t11109 * t25626 - 0.21120586720831816188e-4 * t92456 * t34434 * t22799 - 0.17816121467177433866e-2 * t92471 * t92358 * t100834 + 0.17816121467177433866e-3 * t93122 * t100556 * t100542 - 0.21120586720831816188e-5 * t93129 * t100556 * t101201 + 0.21120586720831816188e-4 * t92357 * t92358 * t930 * t22558 - 0.21120586720831816188e-4 * t92456 * t92358 * t930 * t22820 + 0.7423383944657264111e-4 * t22583 * t22585 * t930 * t1651 + 0.98978452595430188146e-4 * t22583 * t101466 * t930 * t1643 + 0.74233839446572641111e-4 * t22583 * t25692 * t25693 * t1656 + 0.29693535778629056444e-3 * t22583 * t2248 * t423 * t18 * t373 * t384 - 0.4945510644553639738e-5 * t92897 * t25692 * t920 * t1593 * t1632 - 0.17816121467177433866e-2 * t92471 * t34434 * t22598 - 0.12768721675925925926e-1 * t93176 - 0.62424861526748971195e-1 * t93189 - 0.2269994964609053498e-1 * t93192 + 0.29693535778629056444e-3 * t93195 - 0.20429954681481481482e0 * t5598 * t22767 * t25670 + t101498 + 0.64109413167231678975e-5 * t11121 * t5537 * t22652 * t929;
    (t101503,)
}
