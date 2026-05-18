//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 840/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk840<F: Float>(t1937: F, t609: F, t2147: F, t157: F, t2331: F, t524: F, t2152: F, t119: F, t1915: F, t1938: F, t2127: F, t2146: F, t2342: F, t557: F, t616: F, t621: F, t7950: F, t7962: F, t7996: F, t8000: F, t9003: F, t9010: F, t9055: F, t9063: F, t9073: F, t9077: F, t9517: F, t9769: F, t9774: F, t9779: F, t9790: F) -> (F, F, F, F) {
    let t9793 = t609 * t1937;
    let t9794 = t2147 * t9793;
    let t9800 = t2331 * t524 * t157;
    let t9801 = t2152 * t9800;
    let t9804 = -F::new(0.13170898365871023197e1) * t9010 * t557 + t7950 - F::new(0.4336814094102599731e0) * t9517 * t621 - F::new(0.4336814094102599731e0) * t616 * t9769 + F::new(0.4336814094102599731e0) * t2146 * t9774 + F::new(0.34694512752820797848e1) * t9055 + t7962 + F::new(0.65854491829355115987e0) * t119 * t9779 + F::new(0.13170898365871023197e1) * t2127 * t1915 - F::new(0.65854491829355115987e0) * t2127 * t1938 - F::new(0.13170898365871023197e1) * t9063 + F::new(0.17347256376410398924e1) * t9003 * t2342 + F::new(0.17347256376410398924e1) * t2146 * t9790 + F::new(0.8673628188205199462e0) * t2146 * t9794 + t7996 - t8000 - F::new(0.13170898365871023197e1) * t9073 - F::new(0.34694512752820797848e1) * t9077 + F::new(0.8673628188205199462e0) * t2146 * t9801;
    (t9793, t9794, t9801, t9804)
}
