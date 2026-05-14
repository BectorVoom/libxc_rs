//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1407/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1407<F: Float>(t1586: F, t2738: F, t56777: F, t33873: F, t9524: F, t109489: F, t109496: F, t109508: F, t109899: F, t113666: F, t113669: F, t113704: F, t113714: F, t18681: F, t21559: F, t2737: F, t32385: F, t32459: F, t32474: F, t33823: F, t33854: F, t9519: F, t9855: F, t9860: F) -> (F, F) {
    let t115054 = t1586 * t2738 * t56777;
    let t115058 = 0.34722222222222222222e-2 * t9524 * t33873;
    let t115070 = 0.10416666666666666667e-1 * t33854 * t9519 - 0.51588271604938271604e-3 * t113666 - 0.23214722222222222222e-2 * t113669 + 0.11574074074074074074e-2 * t109496 - 0.10722222222222222222e-1 * t109899 * t9855 + 0.20104166666666666667e-2 * t109489 * t9855 + 0.52083333333333333333e-2 * t2737 * t115054 + t115058 + 0.40208333333333333334e-2 * t32474 * t33823 - 0.34722222222222222222e-2 * t2737 * t18681 * t21559 * t32459 + 0.23148148148148148148e-2 * t109508 + 0.20635308641975308642e-2 * t113704 + 0.52083333333333333333e-2 * t9860 * t32385 + 0.51588271604938271605e-2 * t113714;
    (t115054, t115070)
}
