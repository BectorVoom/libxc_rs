//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 858/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk858<F: Float>(t11614: F, t11617: F, t11621: F, t11623: F, t11627: F, t11630: F, t11634: F, t11638: F, t11641: F, t11644: F, t11649: F, t11651: F, t11653: F, t19: F, t2207: F, t10346: F) -> (F, F, F) {
    let t11655 = -0.82073827867876094584e-5 * t11614 - 0.82073827867876094584e-5 * t11617 + 0.11742981196020707897e-5 * t11621 - 0.80732995722642366792e-5 * t11623 + 0.11742981196020707897e-4 * t11627 + 0.73393632475129424356e-6 * t11630 + 0.43497959513593372169e-7 * t11634 + 0.73393632475129424356e-6 * t11638 + 0.11742981196020707897e-4 * t11641 - 0.17098714139140853038e-6 * t11644 - 0.49871249572494154694e-7 * t11649 + 0.15388842725226767735e-5 * t11651 + 0.46971924784082831588e-4 * t11653;
    let t11656 = t2207 * t19;
    let t11657 = t10346 * t11656;
    (t11655, t11656, t11657)
}
