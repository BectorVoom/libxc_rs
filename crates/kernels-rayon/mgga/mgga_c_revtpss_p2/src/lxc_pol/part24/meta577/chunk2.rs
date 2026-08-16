//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1771/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1771(t1196: f64, t12555: f64, t43752: f64, t90357: f64, t6486: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t89839: f64, t89851: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64, t90379: f64, t90384: f64, t90387: f64, t90390: f64) -> (f64, f64, f64) {
    let t90644 = 0.12304822629859687989e5_f64 * t1196 * t43752 * t90357 * t12555;
    let t90670 = t6486 * t6486;
    let t90688 = 0.55570666666666666666e0_f64 * t90379 + 0.13772666666666666666e1_f64 * t68255 - 0.91817777777777777776e0_f64 * t68257 + 0.375102e1_f64 * t90384 + 0.83356e0_f64 * t90387 + 0.27785333333333333334e0_f64 * t90390 + 0.13772666666666666667e1_f64 * t81156 - 0.41318e1_f64 * t81158 - 0.103295e1_f64 * t89839 + 0.309885e1_f64 * t89851 + 0.68863333333333333334e1_f64 * t89865 - 0.123954e2_f64 * t89869 + 0.123954e2_f64 * t89873 + 0.516475e0_f64 * t89877;
    (t90644, t90670, t90688)
}
