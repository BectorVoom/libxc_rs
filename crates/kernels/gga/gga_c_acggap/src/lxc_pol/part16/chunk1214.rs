//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1214/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1214<F: Float>(t159: F, t2155: F, t2347: F, t32030: F, t32033: F, t32036: F, t32043: F, t32048: F, t32052: F, t32054: F, t33566: F, t33747: F, t33767: F, t40595: F, t40620: F, t616: F, t619: F, t7912: F, t8433: F, t9003: F, t9498: F, t9774: F) -> F {
    let t40791 = -F::new(0.26020884564615598386e1) * t7912 * t9498 + F::new(0.4336814094102599731e0) * t7912 * t9774 + F::new(0.8673628188205199462e0) * t9003 * t8433 - t33747 + F::new(0.8673628188205199462e0) * t33566 * t2347 - F::new(0.4336814094102599731e0) * t616 * t619 * t159 * t40595 - F::new(0.65854491829355115987e0) * t32030 - F::new(0.13170898365871023197e1) * t32033 - F::new(0.8673628188205199462e0) * t32036 + F::new(0.26020884564615598386e1) * t32043 + t32048 + t32052 + F::new(0.34694512752820797848e1) * t33767 + t32054 + F::new(0.4336814094102599731e0) * t40620 * t2155;
    t40791
}
