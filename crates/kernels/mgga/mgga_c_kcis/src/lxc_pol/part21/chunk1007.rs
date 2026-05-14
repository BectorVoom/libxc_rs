//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1007/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1007<F: Float>(t26918: F, t26920: F, t26922: F, t26925: F, t26927: F, t26931: F, t26934: F, t26936: F, t26939: F, t26942: F, t26944: F, t26947: F, t27120: F, t2205: F, t3670: F, t11220: F, t11223: F, t11230: F, t1282: F, t1291: F, t26877: F, t26885: F, t26951: F, t27095: F, t27100: F, t27105: F, t3664: F, t3669: F, t437: F, t7812: F, t7823: F) -> (F, F, F) {
    let t27133 = -0.9375e-1 * t26918 + 0.9375e-1 * t26920 + 0.91666666666666666667e0 * t26922 - 0.33333333333333333334e0 * t26925 - 0.21583333333333333334e0 * t26927 + 0.53958333333333333334e-1 * t26931 - 0.1875e0 * t26934 - 0.5e0 * t26936 + 0.125e0 * t26939 + 0.625e-1 * t26942 - 0.20234375e-1 * t26944 - 0.20833333333333333333e-1 * t26947;
    let t27134 = t27120 + t27133;
    let t27136 = t2205 * t3670;
    let t27139 = -t11220 * t2205 + 4.0 * t11223 * t7812 - 6.0 * t11230 * t27136 - t1282 * t27134 - 2.0 * t1291 * t27100 + t27095 * t437 + 4.0 * t27105 * t3669 - 2.0 * t3664 * t7823 - t26877 - t26885 + t26951;
    (t27134, t27136, t27139)
}
