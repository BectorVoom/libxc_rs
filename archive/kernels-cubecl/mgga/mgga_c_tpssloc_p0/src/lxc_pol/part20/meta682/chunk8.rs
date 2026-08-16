//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2582/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2582<F: Float>(t50853: F, t43768: F, t43770: F, t43835: F, t43837: F, t43839: F, t43855: F, t43857: F, t44466: F, t50824: F, t50846: F, t50848: F, t50851: F, t50859: F, t50863: F, t50867: F, t50871: F, t50875: F, t50881: F, t50886: F) -> F {
    let t52313 = F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t50853;
    let t52327 = -F::cast_from(3.0_f64) * t50824 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t50846 + t50848 / F::cast_from(3.0_f64) - t50851 / F::cast_from(6.0_f64) - t52313 - t43768 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t43770 - t44466 + t50859 / F::cast_from(18.0_f64) + F::cast_from(2.0_f64) * t50863 - t50867 - F::cast_from(3.0_f64) * t50871 - t50875 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) * t50881 + t50886 / F::cast_from(6.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t43835 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t43837 + t43839 / F::cast_from(9.0_f64) + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t43855 + F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t43857;
    t52327
}
