//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1300/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1300<F: Float>(t3404: F, t58: F, t22591: F, t554: F, t53: F, t5591: F, t72: F, t100806: F, t5838: F, t100495: F, t22632: F, t23732: F, t26715: F, t100491: F, t1008: F, t100803: F, t2059: F, t22767: F, t23825: F, t23842: F, t26604: F, t26607: F, t26759: F, t3379: F, t5579: F, t5813: F, t6450: F, t8838: F, t94387: F, t94829: F) -> (F, F, F) {
    let t105019 = t58 * t3404;
    let t105021 = t22591 * t105019 * t554;
    let t105031 = t5591 * t72 * t3404 * t53;
    let t105038 = t5838 * t100806;
    let t105044 = t5838 * t100495;
    let t105056 = 0.13335600218518518519e0 * t23732 * t22632 * t26715;
    let t105057 = 0.40006800655555555556e0 * t23732 * t5579 * t72 * t3379 * t554 + 0.90613700826057446696e0 * t8838 * t105021 + 0.53342400874074074074e0 * t26604 * t26759 + 0.53342400874074074074e0 * t5813 * t22767 * t26607 - 0.48335523541469733928e0 * t23842 * t105031 + 0.48335523541469733928e0 * t23825 * t105031 - 0.16299066933744855968e0 * t5838 * t100803 + 0.29634667152263374487e-1 * t105038 - 0.33339000546296296298e-1 * t94829 * t6450 + 0.17780800291358024692e0 * t5838 * t100491 + 0.77791001274691358028e-1 * t105044 - 0.60010200983333333334e0 * t94387 * t5579 * t72 * t1008 * t2059 - 0.10668480174814814815e1 * t23732 * t22767 * t26715 + t105056;
    (t105019, t105021, t105057)
}
