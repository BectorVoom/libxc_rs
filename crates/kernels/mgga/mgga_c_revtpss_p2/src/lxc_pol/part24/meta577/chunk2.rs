//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1771/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1771<F: Float>(t1196: F, t12555: F, t43752: F, t90357: F, t6486: F, t68255: F, t68257: F, t81156: F, t81158: F, t89839: F, t89851: F, t89865: F, t89869: F, t89873: F, t89877: F, t90379: F, t90384: F, t90387: F, t90390: F) -> (F, F, F) {
    let t90644 = F::cast_from(0.12304822629859687989e5_f64) * t1196 * t43752 * t90357 * t12555;
    let t90670 = t6486 * t6486;
    let t90688 = F::cast_from(0.55570666666666666666e0_f64) * t90379 + F::cast_from(0.13772666666666666666e1_f64) * t68255 - F::cast_from(0.91817777777777777776e0_f64) * t68257 + F::new(0.375102e1) * t90384 + F::new(0.83356e0) * t90387 + F::cast_from(0.27785333333333333334e0_f64) * t90390 + F::cast_from(0.13772666666666666667e1_f64) * t81156 - F::new(0.41318e1) * t81158 - F::new(0.103295e1) * t89839 + F::new(0.309885e1) * t89851 + F::cast_from(0.68863333333333333334e1_f64) * t89865 - F::new(0.123954e2) * t89869 + F::new(0.123954e2) * t89873 + F::new(0.516475e0) * t89877;
    (t90644, t90670, t90688)
}
