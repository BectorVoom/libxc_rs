//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2089;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta614(t23437: f64, t3103: f64, t10472: f64, t10474: f64, t10478: f64, t23535: f64, t10948: f64, t23540: f64, t6753: f64, t10961: f64, t6754: f64, t3077: f64, t6764: f64, sigma0: f64, t1937: f64, t607: f64, t6722: f64, t10375: f64, t1942: f64, t1036: f64, t23551: f64, t23562: f64, t343: f64, t83032: f64, t210: f64, t23322: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83046, t83054, t83058, t83061, t83065, t83068, t83071) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2089(t23437, t3103, t10472, t10474, t10478, t23535, t10948, t23540, t6753, t10961, t6754, t3077, t6764, sigma0);
        let (t83075, t83080, t83082, t83085, t83092) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2090(t1937, t607, t6722, t10375, t1942, t1036, t23551, t23562, t343, t83032, t210, t23322);
    (t83046, t83054, t83058, t83061, t83065, t83068, t83071, t83075, t83080, t83082, t83085, t83092)
}
