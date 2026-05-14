//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 880/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk880<F: Float>(t27889: F, t762: F, t17712: F, t2: F, t4: F, t26: F, t3972: F, t6154: F, t13830: F, t1449: F, t27742: F, t675: F, t263: F, t193: F, t13927: F, t6175: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27890 = t762 * t27889;
    let t27892 = t17712 * t2;
    let t27893 = t27892 * t4;
    let t27894 = t27893 * t26;
    let t27897 = t6154 * t3972;
    let t27899 = t13830 * t1449;
    let t27906 = t675 * t27742;
    let t27907 = t27906 * t263;
    let t27908 = t193 * t27907;
    let t27911 = t13927 * t6175;
    (t27890, t27893, t27894, t27897, t27899, t27906, t27907, t27908, t27911)
}
