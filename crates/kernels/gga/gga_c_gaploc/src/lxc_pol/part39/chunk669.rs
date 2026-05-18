//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 669/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk669<F: Float>(t2365: F, t8756: F, t7390: F, t3488: F, t7354: F, t2684: F, t8769: F, t6111: F, t826: F, t825: F, t10813: F, t10815: F, t10819: F, t10823: F, t10825: F, t10830: F, t10831: F, t2033: F, t9789: F, t9799: F, t9803: F, t9809: F) -> (F, F, F, F, F) {
    let t10834 = t2365 * t8756;
    let t10835 = t7390 * t10834;
    let t10836 = F::new(0.14896037479937677779e-1) * t10835;
    let t10837 = t7354 * t3488;
    let t10838 = t2684 * t10837;
    let t10839 = F::new(0.25561950635947166451e0) * t10838;
    let t10840 = t2365 * t8769;
    let t10841 = t6111 * t10840;
    let t10842 = F::new(0.29792074959875355558e-1) * t10841;
    let t10843 = t826 * t3488;
    let t10844 = t825 * t10843;
    let t10845 = F::new(0.25561950635947166451e0) * t10844;
    let t10846 = t10813 - t10815 - t10819 - t10823 - t10825 - t10830 + F::new(0.39722766613167140743e-1) * t2033 * t10831 + t10836 - t10839 + t10842 + t10845 + t9789 - t9799 + t9803 - t9809;
    (t10836, t10839, t10842, t10845, t10846)
}
