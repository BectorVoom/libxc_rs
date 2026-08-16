//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1420/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1420(t34605: f64, t34608: f64, t34630: f64, t37009: f64, t37010: f64, t37011: f64, t37012: f64, t37013: f64, t37014: f64, t37015: f64, t37017: f64, t34641: f64, t34647: f64, t34651: f64, t34654: f64, t34658: f64, t37020: f64, t37022: f64, t37027: f64, t37028: f64, t37029: f64, t37030: f64) -> (f64, f64) {
    let t38593 = -0.18115908419564701086e-6_f64 * t34605 + 0.36231816839129402172e-6_f64 * t34608 + t37009 - t37010 + t37011 + t37012 - t37013 - t37014 - t37015 + 0.19336854506021130164e-7_f64 * t34630 - t37017;
    let t38600 = -t37020 - 0.71958020936198887259e-7_f64 * t34641 + t37022 + 0.95956020918421216159e-7_f64 * t34647 + 0.98332751566569010432e-8_f64 * t34651 + 0.49166375783284505216e-8_f64 * t34654 - 0.65555167711046006954e-8_f64 * t34658 - t37027 - t37028 + t37029 + t37030;
    (t38593, t38600)
}
