//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 822/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk822<F: Float>(t1299: F, t1664: F, t626: F, t71: F, t1301: F, t1300: F, t11154: F, t1303: F, t1603: F, t1626: F, t1701: F, t22673: F, t22787: F, t22791: F, t22796: F, t22800: F, t22804: F, t22807: F, t22811: F, t22814: F, t22819: F, t22822: F, t22826: F, t22829: F, t22834: F, t22839: F, t22842: F, t22846: F, t22850: F, t5514: F, t5538: F, t5540: F, t5611: F, t7854: F, t7860: F, t79: F) -> (F, F, F, F, F) {
    let t22852 = t1664 * t1299;
    let t22855 = t626 * t71;
    let t22856 = t1301 * t22855;
    let t22858 = 0.42562405586419753087e-2 * t1300 * t22856;
    let t22859 = -0.51690243689028715488e-5 * t5538 * t5540 * t11154 - 0.75080154872671831175e-1 * t79 * t22673 + 0.46509801892875584e-2 * t1603 * t22787 + 0.38731446812548799881e-3 * t1603 * t22791 + 0.10560293360415908094e-4 * t22796 * t22800 - 0.42562405586419753086e-2 * t22804 - 0.6384360837962962963e-2 * t5611 * t22807 - 0.85124811172839506173e-2 * t5611 * t22811 + 0.3404992446913580247e-1 * t5611 * t22814 + 0.18164417702296932716e-2 * t22819 * t22822 + 0.46509801892875584e-1 * t22826 * t1626 + 0.22227677429409423704e-2 * t1300 * t1701 * t22829 - 0.46509801892875584e-1 * t22834 * t5514 - 0.60102574844279699039e-6 * t7860 * t22839 - 0.2370952259137005195e-1 * t22842 * t7854 - 0.18727458458024691358e0 * t1300 * t22846 + 0.3404992446913580247e-1 * t22850 - 0.38306165027777777778e-1 * t22852 * t1303 + t22858;
    (t22852, t22855, t22856, t22858, t22859)
}
