//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 803/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk803<F: Float>(t9857: F, t9860: F, t9866: F, t9869: F, t9872: F, t9874: F, t9876: F, t9878: F, t9881: F, t9883: F, t9885: F, t9887: F, t9889: F, t9898: F, t9901: F, t9904: F, t9908: F, t9910: F, t9914: F, t9917: F, t9924: F, t9930: F, t9935: F, t9937: F, t9939: F, t9941: F) -> (F, F) {
    let t10975 = -0.34752370105806885418e-3 * t9857 + 0.51491428373437201896e-5 * t9860 + 0.98478623777692089505e-7 * t9866 + 0.34752370105806885418e-3 * t9869 + 0.17376185052903442709e-3 * t9872 + 0.4637672555408563478e-4 * t9874 - 0.30353495895471971564e-6 * t9876 + 0.53968515702149165441e-6 * t9878 - 0.46497498276882732785e-5 * t9881 + 0.43284943850479925795e-3 * t9883 - 0.43284943850479925795e-3 * t9885 - 0.41223756048076119805e-5 * t9887 + 0.73295838253479341016e-5 * t9889;
    let t10991 = 0.25781643416302550011e-8 * t9898 + 0.42270452978984302532e-6 * t9901 + 0.12380169846338434109e-5 * t9904 - 0.84410248952307505288e-7 * t9908 - 0.16882049790461501058e-6 * t9910 - 0.84410248952307505288e-7 * t9914 - 0.10005428175813516294e-7 * t9917 + 0.20010856351627032588e-7 * t9924 - 0.14591249423061377928e-8 * t9930 + 0.49239311888846044752e-7 * t9935 + 0.21642471925239962898e-3 * t9937 + 0.2318836277704281739e-4 * t9939 + 0.80043425406508130349e-7 * t9941;
    (t10975, t10991)
}
