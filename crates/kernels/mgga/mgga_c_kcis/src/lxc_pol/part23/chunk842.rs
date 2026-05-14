//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 842/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk842<F: Float>(t1409: F, t16533: F, t1951: F, t2642: F, t1650: F, t4035: F, t3815: F, t1445: F, t5789: F, t532: F, t5793: F, t2645: F, t5796: F, t833: F, t3841: F, t5792: F) -> (F, F, F, F, F, F, F, F) {
    let t17028 = t1409 * t16533;
    let t17037 = t1951 * t2642;
    let t17040 = t4035 * t1650;
    let t17041 = t17040 * t3815;
    let t17045 = 0.93706135855523581992e-2 * t1445 * t5789;
    let t17047 = 0.93706135855523581992e-2 * t532 * t5793;
    let t17048 = t1951 * t2645;
    let t17051 = t5796 * t833;
    let t17054 = t5792 * t3841;
    (t17028, t17037, t17041, t17045, t17047, t17048, t17051, t17054)
}
