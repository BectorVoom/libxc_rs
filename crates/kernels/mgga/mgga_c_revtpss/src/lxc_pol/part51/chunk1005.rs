//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1005/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1005<F: Float>(t1903: F, t32211: F, t5673: F, t32206: F, t545: F, t25876: F, t1883: F, t7301: F, t32188: F, t32191: F, t32203: F, t32222: F, t32225: F, t32226: F, t32233: F, t32242: F, t33923: F, t33927: F, t7930: F, t8579: F) -> (F, F, F) {
    let t33930 = t5673 * t32211 * t1903;
    let t33931 = t32206 * t33930;
    let t33935 = t545 * t1903;
    let t33936 = t25876 * t33935;
    let t33939 = t7301 * t1883;
    let t33942 = -t32188 + t32191 - F::cast_from(0.28234466758480466999e-3_f64) * t33923 - t32203 - F::cast_from(0.112937867033921868e-2_f64) * t33927 - F::cast_from(0.28234466758480466999e-3_f64) * t33931 + t32222 - t32225 - F::cast_from(0.17347256376410398924e1_f64) * t32226 * t7930 + F::cast_from(0.17347256376410398924e1_f64) * t8579 * t33936 + F::cast_from(0.8673628188205199462e0_f64) * t32233 * t33939 - t32242;
    (t33930, t33935, t33942)
}
