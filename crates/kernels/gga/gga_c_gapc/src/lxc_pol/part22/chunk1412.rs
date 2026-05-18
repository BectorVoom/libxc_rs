//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1412/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1412<F: Float>(t35071: F, t35074: F, t35080: F, t35083: F, t35090: F, t35093: F, t35095: F, t35097: F, t35077: F, t35086: F, t37184: F, t35108: F) -> (F, F) {
    let t37185 = F::new(0.42206481990611010728e-7) * t35071;
    let t37186 = F::new(0.2698871527777777778e-4) * t35074;
    let t37188 = F::new(0.40483072916666666668e-3) * t35080;
    let t37189 = F::new(0.18310351929594268994e-5) * t35083;
    let t37191 = F::new(0.10298285674687440379e-5) * t35090;
    let t37192 = F::new(0.15716995342493974597e-7) * t35093;
    let t37193 = F::new(0.27012148473991046866e-5) * t35095;
    let t37194 = F::new(0.11594181388521408695e-4) * t35097;
    let t37195 = t37184 - t37185 - t37186 + F::new(0.57970906942607043474e-5) * t35077 - t37188 - t37189 + F::new(0.33460450185846399385e-7) * t35086 + t37191 - t37192 + t37193 + t37194;
    let t37200 = F::new(0.20220636637604418766e-5) * t35108;
    (t37195, t37200)
}
