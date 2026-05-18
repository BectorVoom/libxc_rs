//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1394/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1394<F: Float>(t609: F, t18104: F, t4440: F, t2104: F, t3754: F, t2642: F, t12617: F, t17960: F, t1608: F, t286: F, t1610: F, t833: F, t6171: F) -> (F, F, F, F, F) {
    let t614 = F::new(0.0) < t609;
    let t18105 = t4440 * t18104;
    let t18108 = t2104 * t3754;
    let t18109 = t18108 * t2642;
    let t18110 = t12617 * t18109;
    let t18114 = piecewise3::<f64>(t614, t17960, -t17960);
    let t18115 = t1608 * t18114;
    let t18116 = t286 * t18115;
    let t18119 = t833 * t1610;
    let t18120 = t6171 * t18119;
    (t18105, t18110, t18116, t18119, t18120)
}
