//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 517/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk517<F: Float>(t4224: F, t4227: F, t4233: F, t4238: F, t4242: F, t4298: F, t4302: F, t4307: F, t4310: F, t4314: F, t4316: F, t4318: F, t4551: F, t1611: F, t1620: F, t240: F, t4164: F, t4167: F, t4173: F, t4322: F, t4528: F, t4530: F, t4535: F, t4536: F, t555: F) -> (F, F) {
    let t4564 = 0.20234375e-1 * t4224 - 0.10791666666666666667e0 * t4227 + 0.26979166666666666666e-1 * t4233 - 0.20234375e-1 * t4238 - 0.20833333333333333333e-1 * t4242 + 0.9375e-1 * t4298 - 0.101171875e-1 * t4302 - 0.44965277777777777777e-2 * t4307 - 0.33333333333333333334e0 * t4310 + 0.91666666666666666667e0 * t4314 - 0.5e0 * t4316 + 0.125e0 * t4318;
    let t4565 = t4551 + t4564;
    let t4569 = t4164 - t4167 + t4173 - t4322 + t240 * (-t1611 * t4565 - 2.0 * t1620 * t4530 + t4528 * t555 + 2.0 * t4535 * t4536 - t4164 + t4167 - t4173 + t4322);
    (t4565, t4569)
}
