//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1224/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1224<F: Float>(t2934: F, t633: F, t1221: F, t2146: F, t2394: F, t30023: F, t32331: F, t32335: F, t32990: F, t32992: F, t32997: F, t33000: F, t33008: F, t33751: F, t35324: F, t38015: F, t38018: F, t38019: F, t38033: F, t38036: F, t8400: F, t9427: F) -> F {
    let t38040 = t2934 * t633;
    let t38044 = t38015 - F::new(0.17347256376410398924e1) * t32331 + t38018 + F::new(0.13170898365871023197e1) * t38019 + F::new(0.17347256376410398924e1) * t32335 - F::new(0.34694512752820797848e1) * t32990 - F::new(0.65854491829355115987e0) * t32992 + t32997 + F::new(0.10408353825846239354e2) * t33000 + F::new(0.10408353825846239354e2) * t2146 * t30023 * t2394 * t1221 + F::new(0.13170898365871023197e1) * t33008 - t38033 + t38036 - F::new(0.26020884564615598386e1) * t8400 * t9427 * t33751 + F::new(0.26020884564615598386e1) * t8400 * t38040 * t35324;
    t38044
}
