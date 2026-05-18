//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 625/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk625<F: Float>(t1165: F, t1552: F, t5606: F, t1759: F, t372: F, t4313: F, t360: F, t1181: F, t1891: F, t997: F, t1180: F, t3454: F, t418: F, t4629: F, t4635: F, t4637: F, t4649: F, t4651: F, t4653: F, t5946: F, t5950: F, t5953: F, t5956: F, t5961: F, t5966: F, t5972: F, t5975: F, t5978: F) -> (F, F, F, F) {
    let t5981 = t1165 * t1552 * t5606;
    let t5984 = t1759 * t372;
    let t5986 = t1165 * t4313 * t5984;
    let t5989 = t1759 * t360;
    let t5991 = t1181 * t1552 * t5989;
    let t5996 = t997 * t1891;
    let t5999 = -F::new(0.85748036236139473944e-2) * t418 * t5946 + F::new(0.85748036236139473944e-2) * t418 * t5950 - F::new(0.34299214494455789578e-2) * t5953 - F::new(0.34299214494455789578e-2) * t418 * t5956 - F::new(0.34299214494455789578e-2) * t418 * t5961 - F::new(0.34299214494455789578e-2) * t418 * t5966 + F::new(0.85748036236139473944e-3) * t5972 + F::new(0.42874018118069736972e-2) * t418 * t5975 + F::new(0.42874018118069736972e-3) * t5978 + F::new(0.85748036236139473944e-3) * t1180 * t5981 - F::new(0.25724410870841842183e-2) * t1180 * t5986 + F::new(0.17149607247227894789e-2) * t1180 * t5991 + t4629 - F::new(35.0) / F::new(108.0) * t4635 - F::new(35.0) / F::new(216.0) * t4637 + t4649 + t4651 + t4653 + F::new(0.40015750243531754507e-2) * t5996 - F::new(0.42874018118069736972e-3) * t3454;
    (t5981, t5986, t5991, t5999)
}
