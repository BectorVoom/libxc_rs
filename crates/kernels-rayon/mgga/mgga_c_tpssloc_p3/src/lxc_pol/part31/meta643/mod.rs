//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1911;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1912;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta643(t10109: f64, t1888: f64, t23270: f64, t5636: f64, t865: f64, t25045: f64, t86873: f64, t214: f64, t5631: f64, t1880: f64, t6572: f64, t22986: f64, t5657: f64, t776: f64, t857: f64, t258: f64, t5527: f64, t87642: f64, t6552: f64, t7479: f64, t87782: f64, t2717: f64, t5544: f64, t25038: f64, t23237: f64, t28294: f64, t28267: f64, t82159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98122, t98125, t98133, t98135, t98148) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1911(t10109, t1888, t23270, t5636, t865, t25045, t86873, t214, t5631, t1880, t6572, t22986, t5657, t776, t857);
        let (t98153, t98158, t98164, t98169) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1912(t23270, t258, t5527, t776, t87642, t6552, t7479, t87782, t2717, t5636, t22986, t5544);
        let (t98172, t98181, t98189, t98192) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1913(t23270, t25038, t776, t98169, t1888, t2717, t5657, t865, t1880, t23237, t28294, t22986, t28267, t82159);
    (t98122, t98125, t98133, t98135, t98148, t98153, t98158, t98164, t98172, t98181, t98189, t98192)
}
