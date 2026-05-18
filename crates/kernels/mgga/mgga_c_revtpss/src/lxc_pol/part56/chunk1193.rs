//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1193/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1193<F: Float>(t12916: F, t34944: F, t8946: F, t104527: F, t3736: F, t8937: F, t1032: F, t1203: F, t1214: F, t124706: F, t124744: F, t124887: F, t125048: F, t125059: F, t131631: F, t2150: F, t33436: F, t33441: F, t33461: F, t33462: F, t33477: F, t33478: F, t33487: F, t34908: F, t34931: F, t34939: F, t3555: F, t473: F, t5215: F, t5245: F, t5407: F, t5497: F, t7652: F, t8925: F, t8931: F) -> F {
    let t132018 = t8946 * t34944 * t12916;
    let t132032 = t8937 * t104527 * t3736;
    let t132047 = F::new(0.56468933516960933998e-3) * t3555 * t1032 * t8925 * t34931 - F::new(0.17135921299530705785e1) * t33477 * t33478 * t8931 * t5497 + F::new(0.34694512752820797848e1) * t124887 * t7652 * t131631 + F::new(0.66110807482757352569e-3) * t132018 - F::new(0.37645955677973955998e-3) * t125059 + F::new(0.34271842599061411569e1) * t33461 * t33462 * t34908 * t1203 - F::new(0.17347256376410398924e1) * t33436 * t2150 * t473 * t5215 + F::new(0.24791552806034007214e-3) * t124744 * t5407 + F::new(0.11423947533020470523e1) * t132032 * t33487 + F::new(0.17347256376410398924e1) * t33441 * t2150 * t473 * t5245 - F::new(0.51407763898592117355e1) * t124706 * t33462 * t34939 * t1203 + F::new(0.6854368519812282314e1) * t125048 * t33462 * t34939 * t1214;
    t132047
}
