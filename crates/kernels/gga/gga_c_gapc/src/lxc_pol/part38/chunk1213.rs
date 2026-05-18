//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1213/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1213<F: Float>(t13853: F, t169: F, t21204: F, t4043: F, t519: F, t11430: F, t3060: F, t8716: F, t34995: F, t35001: F, t35003: F, t35005: F, t35007: F, t35010: F, t35013: F, t35016: F, t35019: F) -> F {
    let t35024 = t169 * t21204 * t4043 * t519 * t13853;
    let t35027 = t3060 * t11430 * t8716;
    let t35029 = F::new(0.10793703140429833089e-5) * t34995 - F::new(0.12187980608940473897e-4) * t35001 + F::new(0.4637672555408563478e-4) * t35003 - F::new(0.4637672555408563478e-4) * t35005 + F::new(0.49522272202316919254e-5) * t35007 - F::new(0.22510123728325872388e-6) * t35010 - F::new(0.38647271295071362318e-6) * t35013 - F::new(0.6629778687778673199e-7) * t35016 + F::new(0.40022999988963401106e-8) * t35019 - F::new(0.24877751768706223874e-6) * t35024 + F::new(0.30775559784820528656e-8) * t35027;
    t35029
}
