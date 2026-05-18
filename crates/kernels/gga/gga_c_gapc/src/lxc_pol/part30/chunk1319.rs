//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1319/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1319<F: Float>(t34808: F, t34811: F, t34813: F, t34820: F, t34822: F, t34824: F, t34826: F, t34830: F, t34832: F, t34834: F, t34839: F, t34873: F, t34876: F, t34878: F, t34881: F, t34884: F, t34886: F, t34889: F, t34891: F, t34894: F, t34897: F, t34900: F) -> (F, F) {
    let t38294 = F::new(0.15458908518028544927e-5) * t34808 - F::new(0.2748593934505475288e-5) * t34811 + F::new(0.54868426587794313947e-6) * t34813 - F::new(0.41825562732307999229e-9) * t34820 - F::new(0.72032395930642791642e-6) * t34822 + F::new(0.16893513245878347078e-5) * t34824 - F::new(0.10120768229166666668e-4) * t34826 + F::new(0.86880925264517213544e-4) * t34830 - F::new(0.16009199995585360443e-6) * t34832 + F::new(0.12807359996468288354e-5) * t34834 + F::new(0.40441273275208837532e-5) * t34839;
    let t38321 = -F::new(0.7246363367825880434e-6) * t34873 - F::new(0.89958769611019890143e-8) * t34876 + F::new(0.22393919123163518362e-6) * t34878 + F::new(0.33111854833537703651e-5) * t34881 - F::new(0.39333100626627604174e-7) * t34884 + F::new(0.56275309320814680971e-7) * t34886 - F::new(0.10477996894995983065e-7) * t34889 - F::new(0.82537120337194865424e-4) * t34891 + F::new(0.18629878479302857891e-8) * t34894 + F::new(0.5691280480400994668e-7) * t34897 - F::new(0.1011909669415296852e-6) * t34900;
    (t38294, t38321)
}
