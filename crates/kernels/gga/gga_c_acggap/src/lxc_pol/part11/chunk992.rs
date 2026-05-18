//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 992/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk992<F: Float>(t5299: F, t615: F, t310: F, t8995: F, t1221: F, t2146: F, t2341: F, t30023: F, t31965: F, t31976: F, t31978: F, t31984: F, t33635: F, t33644: F, t33648: F, t33651: F, t33656: F, t621: F, t7912: F, t7931: F, t7932: F, t8014: F, t8428: F, t8441: F, t9003: F, t9033: F) -> F {
    let t33658 = t615 * t5299;
    let t33662 = F::new(0.13170898365871023197e1) * t310 * t8995;
    let t33666 = F::new(0.8673628188205199462e0) * t7912 * t8428 + F::new(0.69389025505641595696e1) * t31976 + F::new(0.34694512752820797848e1) * t33635 + F::new(0.10408353825846239354e2) * t2146 * t30023 * t2341 * t1221 - F::new(0.17347256376410398924e1) * t31965 * t8441 + F::new(0.17347256376410398924e1) * t7931 * t9033 * t33644 - F::new(0.65854491829355115987e0) * t33648 - F::new(0.8673628188205199462e0) * t7931 * t7932 * t33651 - F::new(0.17347256376410398924e1) * t31978 + F::new(0.65854491829355115987e0) * t33656 - F::new(0.4336814094102599731e0) * t33658 * t621 + t33662 + F::new(0.8673628188205199462e0) * t9003 * t8014 + F::new(0.65854491829355115987e0) * t31984;
    t33666
}
