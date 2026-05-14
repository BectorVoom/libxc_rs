//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 721/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk721<F: Float>(t1045: F, t5510: F, t1043: F, t1432: F, t2982: F, t2980: F, t8927: F, t8930: F, t8933: F, t8935: F, t8938: F, t8941: F, t8943: F, t8945: F, t8952: F, t8961: F, t8963: F) -> (F, F, F, F) {
    let t8965 = t1045 * t5510;
    let t8966 = t1043 * t8965;
    let t8968 = t2982 * t1432;
    let t8969 = t2980 * t8968;
    let t8971 = -0.19323635647535681159e-7 * t8927 + 0.72463633678258804342e-6 * t8930 + 0.12357942809624928455e-3 * t8933 + 0.37109483506944444446e-4 * t8935 + 0.69504740211613770836e-4 * t8938 - 0.37073828428874785365e-3 * t8941 + 0.83413693878529023666e-4 * t8943 - 0.67471788194444444446e-5 * t8945 - 0.98396357783564814818e-6 * t8952 + 0.43048406530309606484e-6 * t8961 + 0.12357942809624928455e-3 * t8963 + 0.14492726735651760868e-5 * t8966 + 0.20241536458333333334e-4 * t8969;
    (t8965, t8966, t8969, t8971)
}
