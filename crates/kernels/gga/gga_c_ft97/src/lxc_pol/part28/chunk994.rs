//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 994/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk994<F: Float>(t2185: F, t23657: F, t27123: F, t5900: F, t139212: F, t139224: F, t27152: F, t32899: F, t139213: F, t139214: F, t27091: F, t147944: F, t7239: F, t7366: F, t7369: F, t1369: F, t34839: F, t376: F) -> (F, F, F, F, F) {
    let t148435 = t23657 * t2185 * t5900 * t27123;
    let t148439 = t139212 * t139224 * t32899 * t27152;
    let t148443 = t139212 * t139213 * t139214 * t27091;
    let t148446 = t7366 * t7239 * t7369 * t147944;
    let t148449 = t1369 * t376 * t34839;
    (t148435, t148439, t148443, t148446, t148449)
}
