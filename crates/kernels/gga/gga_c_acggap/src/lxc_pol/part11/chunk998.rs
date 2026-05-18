//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 998/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk998<F: Float>(t2131: F, t2147: F, t2341: F, t847: F, t2331: F, t862: F, t865: F, t32030: F, t32033: F, t32036: F, t32039: F, t32043: F, t32048: F, t32052: F, t32054: F, t32057: F, t32061: F, t32064: F, t32069: F, t557: F, t7917: F, t9003: F) -> F {
    let t33767 = t2131 * t2147 * t2341 * t847;
    let t33771 = t862 * t2331 * t865;
    let t33775 = -F::new(0.13170898365871023197e1) * t32030 - F::new(0.26341796731742046394e1) * t32033 + F::new(0.8673628188205199462e0) * t9003 * t7917 - F::new(0.17347256376410398924e1) * t32036 - F::new(0.8673628188205199462e0) * t32039 + F::new(0.52041769129231196772e1) * t32043 - F::new(0.65854491829355115987e0) * t32069 * t557 + t32048 + t32052 + F::new(0.17347256376410398924e1) * t33767 + t32054 - F::new(0.52041769129231196772e1) * t32057 + F::new(0.13170898365871023197e1) * t33771 + F::new(0.10408353825846239354e2) * t32061 - F::new(0.8673628188205199462e0) * t32064;
    t33775
}
