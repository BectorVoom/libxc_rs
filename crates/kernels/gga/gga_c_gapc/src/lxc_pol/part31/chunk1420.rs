//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1420/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1420<F: Float>(t34605: F, t34608: F, t34630: F, t37009: F, t37010: F, t37011: F, t37012: F, t37013: F, t37014: F, t37015: F, t37017: F, t34641: F, t34647: F, t34651: F, t34654: F, t34658: F, t37020: F, t37022: F, t37027: F, t37028: F, t37029: F, t37030: F) -> (F, F) {
    let t38593 = -F::new(0.18115908419564701086e-6) * t34605 + F::new(0.36231816839129402172e-6) * t34608 + t37009 - t37010 + t37011 + t37012 - t37013 - t37014 - t37015 + F::new(0.19336854506021130164e-7) * t34630 - t37017;
    let t38600 = -t37020 - F::new(0.71958020936198887259e-7) * t34641 + t37022 + F::new(0.95956020918421216159e-7) * t34647 + F::new(0.98332751566569010432e-8) * t34651 + F::new(0.49166375783284505216e-8) * t34654 - F::new(0.65555167711046006954e-8) * t34658 - t37027 - t37028 + t37029 + t37030;
    (t38593, t38600)
}
